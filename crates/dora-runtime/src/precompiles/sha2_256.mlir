module {
  func.func @sha2_256(%input: memref<?xi8>) -> memref<32xi8> {
    %c32 = arith.constant 32 : index
    %result = memref.alloc() : memref<32xi8>
    %c0 = arith.constant 0 : index
    %c1 = arith.constant 1 : index
    scf.for %i = %c0 to %c32 step %c1 {
      %val = arith.constant 0 : i8
      memref.store %val, %result[%i] : memref<32xi8>
    }
    return %result : memref<32xi8>
  }
}
