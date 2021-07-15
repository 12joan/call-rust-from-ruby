require 'ffi'

module ExternalFunctions
  extend FFI::Library
  ffi_lib '/fibonacci/libfibonacci.so'
  attach_function :fibonacci, [:int], :int
end

puts ExternalFunctions.fibonacci(42)
