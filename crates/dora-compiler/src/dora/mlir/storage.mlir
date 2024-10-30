//===----------------------------------------------------------------------===//
// Module Declaration
//===----------------------------------------------------------------------===//
module {
  //===----------------------------------------------------------------------===//
  // Type Definitions
  //===----------------------------------------------------------------------===//

  // Define a struct for U256Map
  // !U256Map = memref<?x3xi256>  // capacity [key, value, used]

  // Create a new U256Map
  // @param capacity: initial capacity of the map
  // @return: new U256Map instance
  // Create a new U256Map
  // @param capacity: initial capacity of the map
  // @return: new U256Map instance
  func.func @dora_u256_map_new(%capacity: index) -> memref<?x3xi256> {
    %c0 = arith.constant 0 : index
    %c1 = arith.constant 1 : index
    %c2 = arith.constant 2 : index
    %zero_u256 = arith.constant 0 : i256

    // Allocate memory for entries array
    %entries = memref.alloc(%capacity) : memref<?x3xi256>

    // Initialize all entries with default values
    scf.for %i = %c0 to %capacity step %c1 {
      // Store zero key
      memref.store %zero_u256, %entries[%i, %c0] : memref<?x3xi256>
      // Store zero value
      memref.store %zero_u256, %entries[%i, %c1] : memref<?x3xi256>
      // Store used flag, default is false
      memref.store %zero_u256, %entries[%i, %c2] : memref<?x3xi256>
    }

    return %entries : memref<?x3xi256>
  }

  // Simple hash function
  func.func @dora_u256_map_simple_hash(%entries: memref<?x3xi256>, %key: i256) -> index {
    %c0 = arith.constant 0 : index
    // Get capacity from memref dimension
    %capacity = memref.dim %entries, %c0 : memref<?x3xi256>
    %capacity_i256 = arith.index_cast %capacity : index to i256
    %final_hash = llvm.urem %key, %capacity_i256 : i256
    // Convert back to index
    %result = arith.index_cast %final_hash : i256 to index
    return %result : index
  }

  // Hash function
  func.func @dora_u256_map_hash(%entries: memref<?x3xi256>, %key: i256) -> index {
    %c0 = arith.constant 0 : index
    %c1_i256 = arith.constant 1 : i256
    // Convert key to ptr for hashing
    %key_ptr = llvm.alloca %c1_i256 x i256 : (i256) -> !llvm.ptr
    llvm.store %key, %key_ptr : i256, !llvm.ptr
    // Get capacity from memref dimension
    %capacity = memref.dim %entries, %c0 : memref<?x3xi256>
    %capacity_i64 = arith.index_cast %capacity : index to i64
    %vec = llvm.load %key_ptr : !llvm.ptr -> vector<4 x i64>
    %zero = llvm.mlir.constant(dense<0> : vector<4 x i64>) : vector<4 x i64>
    %xored = llvm.xor %vec, %zero : vector<4 x i64>
    %reduced = "llvm.vector.reduce.xor"(%xored) : (vector<4 x i64>) -> i64
    %final_hash = llvm.urem %reduced, %capacity_i64 : i64
    // Convert back to index
    %result = arith.index_cast %final_hash : i64 to index
    return %result : index
  }

  func.func @dora_u256_map_insert(%entries: memref<?x3xi256>, %key: i256, %value: i256) -> memref<?x3xi256> {
    %c0 = arith.constant 0 : index
    %c1 = arith.constant 1 : index
    %c2 = arith.constant 2 : index
    %true = arith.constant 1 : i256
    %false = arith.constant 0 : i256

    // Get capacity
    %capacity = memref.dim %entries, %c0 : memref<?x3xi256>

    // Get initial index from hash
    %idx = call @dora_u256_map_hash(%entries, %key) : (memref<?x3xi256>, i256) -> index
    %curr_idx = memref.alloc() : memref<1xindex>
    memref.store %idx, %curr_idx[%c0] : memref<1xindex>

    // Linear probing loop
    scf.while: () -> () {
      // Load current index
      %current = memref.load %curr_idx[%c0] : memref<1xindex>

      // Check if slot is used
      %used = memref.load %entries[%current, %c2] : memref<?x3xi256>
      %is_used = arith.cmpi eq, %used, %true : i256

      scf.if %is_used {
        // Slot is used, check if key matches
        %stored_key = memref.load %entries[%current, %c0] : memref<?x3xi256>
        %key_matches = arith.cmpi eq, %stored_key, %key : i256

        scf.if %key_matches {
          // Update value if key matches
          memref.store %value, %entries[%current, %c1] : memref<?x3xi256>
          scf.yield
        } else {
          // Continue probing
          %next_idx = arith.addi %current, %c1 : index
          %wrapped_idx = arith.remsi %next_idx, %capacity : index
          memref.store %wrapped_idx, %curr_idx[%c0] : memref<1xindex>
          scf.yield
        }
      } else {
        // Found empty slot, insert here
        memref.store %key, %entries[%current, %c0] : memref<?x3xi256>
        memref.store %value, %entries[%current, %c1] : memref<?x3xi256>
        memref.store %true, %entries[%current, %c2] : memref<?x3xi256>
        scf.yield
      }

      scf.condition(%is_used)
    } do {
      scf.yield
    }

    // Cleanup
    memref.dealloc %curr_idx : memref<1xindex>

    return %entries : memref<?x3xi256>
  }

  // Get function
  func.func @dora_u256_map_get(%entries: memref<?x3xi256>, %key: i256) -> (i256, i1) {
    %c0 = arith.constant 0 : index
    %c1 = arith.constant 1 : index
    %c2 = arith.constant 2 : index
    %true = arith.constant 1 : i256
    %default_result = arith.constant 0 : i256
    %false_i1 = arith.constant false
    %true_i1 = arith.constant true

    // Get capacity
    %capacity = memref.dim %entries, %c0 : memref<?x3xi256>

    // Get initial index from hash
    %idx = call @dora_u256_map_hash(%entries, %key) : (memref<?x3xi256>, i256) -> index
    %curr_idx = memref.alloc() : memref<1xindex>
    memref.store %idx, %curr_idx[%c0] : memref<1xindex>

    // Result memory allocation
    %result = memref.alloc() : memref<1xi256>
    %found = memref.alloc() : memref<1xi1>
    memref.store %default_result, %result[%c0] : memref<1xi256>
    memref.store %false_i1, %found[%c0] : memref<1xi1>

    // Linear probing loop
    scf.while: () -> () {
      // Load current index
      %current = memref.load %curr_idx[%c0] : memref<1xindex>

      // Check if slot is used
      %used = memref.load %entries[%current, %c2] : memref<?x3xi256>
      %is_used = arith.cmpi eq, %used, %true : i256

      scf.if %is_used {
        // Check if key matches
        %stored_key = memref.load %entries[%current, %c0] : memref<?x3xi256>
        %key_matches = arith.cmpi eq, %stored_key, %key : i256

        scf.if %key_matches {
          // Found the key, store the value in result
          %value = memref.load %entries[%current, %c1] : memref<?x3xi256>
          memref.store %value, %result[%c0] : memref<1xi256>
          memref.store %true_i1, %found[%c0] : memref<1xi1>
        } else {
          // Continue probing
          %next_idx = arith.addi %current, %c1 : index
          %wrapped_idx = arith.remsi %next_idx, %capacity : index
          memref.store %wrapped_idx, %curr_idx[%c0] : memref<1xindex>
        }
        scf.yield
      } else {
        // Found empty slot, key doesn't exist
        scf.yield
      }

      // Continue while slot is used and key hasn't been found
      %continue = scf.if %is_used -> i1 {
        %stored_key = memref.load %entries[%current, %c0] : memref<?x3xi256>
        %key_matches = arith.cmpi ne, %stored_key, %key : i256
        scf.yield %key_matches : i1
      } else {
        %false = arith.constant false
        scf.yield %false : i1
      }

      scf.condition(%continue)
    } do {
      scf.yield
    }

    // Get final result
    %final_result = memref.load %result[%c0] : memref<1xi256>
    %final_found = memref.load %found[%c0] : memref<1xi1>

    // Cleanup
    memref.dealloc %curr_idx : memref<1xindex>
    memref.dealloc %result : memref<1xi256>
    memref.dealloc %found : memref<1xi1>

    return %final_result, %final_found : i256, i1
  }
}
