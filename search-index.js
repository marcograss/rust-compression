var N = null;var searchIndex = {};
searchIndex["compression"]={"doc":"rust-compression","items":[[0,"prelude","compression","",N,N],[3,"BZip2Decoder","compression::prelude","",N,N],[3,"BZip2Encoder","","",N,N],[3,"Deflater","","",N,N],[3,"Inflater","","",N,N],[3,"GZipDecoder","","",N,N],[3,"GZipEncoder","","",N,N],[3,"LzhufDecoder","","",N,N],[3,"LzhufEncoder","","",N,N],[3,"ZlibDecoder","","",N,N],[3,"ZlibEncoder","","",N,N],[3,"DecodeIterator","","",N,N],[3,"EncodeIterator","","",N,N],[4,"Action","","",N,N],[13,"Run","","",0,N],[13,"Flush","","",0,N],[13,"Finish","","",0,N],[4,"BZip2Error","","",N,N],[13,"DataError","","",1,N],[13,"DataErrorMagicFirst","","",1,N],[13,"DataErrorMagic","","",1,N],[13,"UnexpectedEof","","",1,N],[13,"Unexpected","","",1,N],[4,"LzhufMethod","","",N,N],[13,"Lh4","","",2,N],[13,"Lh5","","",2,N],[13,"Lh6","","",2,N],[13,"Lh7","","",2,N],[4,"CompressionError","","",N,N],[13,"DataError","","",3,N],[13,"UnexpectedEof","","",3,N],[13,"Unexpected","","",3,N],[8,"DecodeExt","","",N,N],[10,"decode","","",4,[[["self"],["e"]],["decodeiterator"]]],[8,"Decoder","","",N,N],[16,"Error","","",5,N],[16,"Output","","",5,N],[10,"next","","",5,[[["self"],["r"]],["result",["option"]]]],[11,"iter","","",5,[[["self"],["r"]],["decodeiterator"]]],[8,"EncodeExt","","",N,N],[10,"encode","","",6,[[["self"],["e"],["action"]],["encodeiterator"]]],[8,"Encoder","","",N,N],[16,"Error","","",7,N],[10,"next","","",7,[[["self"],["i"],["action"]],["option",["result"]]]],[11,"from","","",0,[[["t"]],["t"]]],[11,"into","","",0,[[["self"]],["u"]]],[11,"to_owned","","",0,[[["self"]],["t"]]],[11,"clone_into","","",0,N],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"try_into","","",0,[[["self"]],["result"]]],[11,"get_type_id","","",0,[[["self"]],["typeid"]]],[11,"from","","",3,[[["t"]],["t"]]],[11,"into","","",3,[[["self"]],["u"]]],[11,"to_string","","",3,[[["self"]],["string"]]],[11,"to_owned","","",3,[[["self"]],["t"]]],[11,"clone_into","","",3,N],[11,"try_from","","",3,[[["u"]],["result"]]],[11,"borrow","","",3,[[["self"]],["t"]]],[11,"borrow_mut","","",3,[[["self"]],["t"]]],[11,"try_into","","",3,[[["self"]],["result"]]],[11,"get_type_id","","",3,[[["self"]],["typeid"]]],[11,"from","","",8,[[["t"]],["t"]]],[11,"into","","",8,[[["self"]],["u"]]],[11,"into_iter","","",8,[[["self"]],["i"]]],[11,"try_from","","",8,[[["u"]],["result"]]],[11,"borrow","","",8,[[["self"]],["t"]]],[11,"borrow_mut","","",8,[[["self"]],["t"]]],[11,"try_into","","",8,[[["self"]],["result"]]],[11,"get_type_id","","",8,[[["self"]],["typeid"]]],[11,"from","","",9,[[["t"]],["t"]]],[11,"into","","",9,[[["self"]],["u"]]],[11,"into_iter","","",9,[[["self"]],["i"]]],[11,"try_from","","",9,[[["u"]],["result"]]],[11,"borrow","","",9,[[["self"]],["t"]]],[11,"borrow_mut","","",9,[[["self"]],["t"]]],[11,"try_into","","",9,[[["self"]],["result"]]],[11,"get_type_id","","",9,[[["self"]],["typeid"]]],[11,"from","","",10,[[["t"]],["t"]]],[11,"into","","",10,[[["self"]],["u"]]],[11,"try_from","","",10,[[["u"]],["result"]]],[11,"borrow","","",10,[[["self"]],["t"]]],[11,"borrow_mut","","",10,[[["self"]],["t"]]],[11,"try_into","","",10,[[["self"]],["result"]]],[11,"get_type_id","","",10,[[["self"]],["typeid"]]],[11,"from","","",11,[[["t"]],["t"]]],[11,"into","","",11,[[["self"]],["u"]]],[11,"try_from","","",11,[[["u"]],["result"]]],[11,"borrow","","",11,[[["self"]],["t"]]],[11,"borrow_mut","","",11,[[["self"]],["t"]]],[11,"try_into","","",11,[[["self"]],["result"]]],[11,"get_type_id","","",11,[[["self"]],["typeid"]]],[11,"from","","",1,[[["t"]],["t"]]],[11,"into","","",1,[[["self"]],["u"]]],[11,"to_string","","",1,[[["self"]],["string"]]],[11,"to_owned","","",1,[[["self"]],["t"]]],[11,"clone_into","","",1,N],[11,"try_from","","",1,[[["u"]],["result"]]],[11,"borrow","","",1,[[["self"]],["t"]]],[11,"borrow_mut","","",1,[[["self"]],["t"]]],[11,"try_into","","",1,[[["self"]],["result"]]],[11,"get_type_id","","",1,[[["self"]],["typeid"]]],[11,"from","","",12,[[["t"]],["t"]]],[11,"into","","",12,[[["self"]],["u"]]],[11,"try_from","","",12,[[["u"]],["result"]]],[11,"borrow","","",12,[[["self"]],["t"]]],[11,"borrow_mut","","",12,[[["self"]],["t"]]],[11,"try_into","","",12,[[["self"]],["result"]]],[11,"get_type_id","","",12,[[["self"]],["typeid"]]],[11,"from","","",13,[[["t"]],["t"]]],[11,"into","","",13,[[["self"]],["u"]]],[11,"try_from","","",13,[[["u"]],["result"]]],[11,"borrow","","",13,[[["self"]],["t"]]],[11,"borrow_mut","","",13,[[["self"]],["t"]]],[11,"try_into","","",13,[[["self"]],["result"]]],[11,"get_type_id","","",13,[[["self"]],["typeid"]]],[11,"from","","",2,[[["t"]],["t"]]],[11,"into","","",2,[[["self"]],["u"]]],[11,"try_from","","",2,[[["u"]],["result"]]],[11,"borrow","","",2,[[["self"]],["t"]]],[11,"borrow_mut","","",2,[[["self"]],["t"]]],[11,"try_into","","",2,[[["self"]],["result"]]],[11,"get_type_id","","",2,[[["self"]],["typeid"]]],[11,"from","","",14,[[["t"]],["t"]]],[11,"into","","",14,[[["self"]],["u"]]],[11,"try_from","","",14,[[["u"]],["result"]]],[11,"borrow","","",14,[[["self"]],["t"]]],[11,"borrow_mut","","",14,[[["self"]],["t"]]],[11,"try_into","","",14,[[["self"]],["result"]]],[11,"get_type_id","","",14,[[["self"]],["typeid"]]],[11,"from","","",15,[[["t"]],["t"]]],[11,"into","","",15,[[["self"]],["u"]]],[11,"try_from","","",15,[[["u"]],["result"]]],[11,"borrow","","",15,[[["self"]],["t"]]],[11,"borrow_mut","","",15,[[["self"]],["t"]]],[11,"try_into","","",15,[[["self"]],["result"]]],[11,"get_type_id","","",15,[[["self"]],["typeid"]]],[11,"from","","",16,[[["t"]],["t"]]],[11,"into","","",16,[[["self"]],["u"]]],[11,"try_from","","",16,[[["u"]],["result"]]],[11,"borrow","","",16,[[["self"]],["t"]]],[11,"borrow_mut","","",16,[[["self"]],["t"]]],[11,"try_into","","",16,[[["self"]],["result"]]],[11,"get_type_id","","",16,[[["self"]],["typeid"]]],[11,"from","","",17,[[["t"]],["t"]]],[11,"into","","",17,[[["self"]],["u"]]],[11,"try_from","","",17,[[["u"]],["result"]]],[11,"borrow","","",17,[[["self"]],["t"]]],[11,"borrow_mut","","",17,[[["self"]],["t"]]],[11,"try_into","","",17,[[["self"]],["result"]]],[11,"get_type_id","","",17,[[["self"]],["typeid"]]],[11,"from","","",18,[[["t"]],["t"]]],[11,"into","","",18,[[["self"]],["u"]]],[11,"try_from","","",18,[[["u"]],["result"]]],[11,"borrow","","",18,[[["self"]],["t"]]],[11,"borrow_mut","","",18,[[["self"]],["t"]]],[11,"try_into","","",18,[[["self"]],["result"]]],[11,"get_type_id","","",18,[[["self"]],["typeid"]]],[11,"from","","",19,[[["t"]],["t"]]],[11,"into","","",19,[[["self"]],["u"]]],[11,"try_from","","",19,[[["u"]],["result"]]],[11,"borrow","","",19,[[["self"]],["t"]]],[11,"borrow_mut","","",19,[[["self"]],["t"]]],[11,"try_into","","",19,[[["self"]],["result"]]],[11,"get_type_id","","",19,[[["self"]],["typeid"]]],[11,"next","","",10,[[["self"],["r"]],["result",["option"]]]],[11,"next","","",12,[[["self"],["r"]],["result",["option"]]]],[11,"next","","",14,[[["self"],["r"]],["result",["option"]]]],[11,"next","","",16,[[["self"],["r"]],["result",["option"]]]],[11,"next","","",18,[[["self"],["r"]],["result",["option"]]]],[11,"next","","",11,[[["self"],["i"],["action"]],["option",["result"]]]],[11,"next","","",13,[[["self"],["i"],["action"]],["option",["result"]]]],[11,"next","","",15,[[["self"],["i"],["action"]],["option",["result"]]]],[11,"next","","",17,[[["self"],["i"],["action"]],["option",["result"]]]],[11,"next","","",19,[[["self"],["i"],["action"]],["option",["result"]]]],[11,"clone","","",0,[[["self"]],["action"]]],[11,"clone","","",3,[[["self"]],["compressionerror"]]],[11,"clone","","",1,[[["self"]],["bzip2error"]]],[11,"from","","",3,[[["bzip2error"]],["self"]]],[11,"next","","",8,[[["self"]],["option",["result"]]]],[11,"next","","",9,[[["self"]],["option",["result"]]]],[11,"default","","",10,[[],["self"]]],[11,"default","","",11,[[],["self"]]],[11,"default","","",12,[[],["self"]]],[11,"default","","",13,[[],["self"]]],[11,"default","","",16,[[],["self"]]],[11,"default","","",17,[[],["self"]]],[11,"default","","",18,[[],["zlibdecoder"]]],[11,"default","","",19,[[],["self"]]],[11,"eq","","",0,[[["self"],["action"]],["bool"]]],[11,"eq","","",3,[[["self"],["compressionerror"]],["bool"]]],[11,"eq","","",1,[[["self"],["bzip2error"]],["bool"]]],[11,"fmt","","",3,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",1,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",0,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",3,[[["self"],["formatter"]],["result"]]],[11,"fmt","","",1,[[["self"],["formatter"]],["result"]]],[11,"description","","",3,[[["self"]],["str"]]],[11,"cause","","",3,[[["self"]],["option",["error"]]]],[11,"description","","",1,[[["self"]],["str"]]],[11,"cause","","",1,[[["self"]],["option",["error"]]]],[11,"iter","","",5,[[["self"],["r"]],["decodeiterator"]]],[11,"new","","",10,[[],["self"]]],[11,"new","","",11,[[["usize"]],["self"]]],[11,"new","","",12,[[],["self"]]],[11,"with_dict","","",12,N],[11,"new","","",13,[[],["self"]]],[11,"with_dict","","",13,N],[11,"new","","",14,[[["lzhufmethod"]],["self"]]],[11,"new","","",15,[[["lzhufmethod"]],["self"]]],[11,"new","","",16,[[],["self"]]],[11,"new","","",17,[[],["self"]]],[11,"new","","",18,[[],["self"]]],[11,"with_dict","","",18,N],[11,"new","","",19,[[],["self"]]],[11,"with_dict","","",19,N]],"paths":[[4,"Action"],[4,"BZip2Error"],[4,"LzhufMethod"],[4,"CompressionError"],[8,"DecodeExt"],[8,"Decoder"],[8,"EncodeExt"],[8,"Encoder"],[3,"DecodeIterator"],[3,"EncodeIterator"],[3,"BZip2Decoder"],[3,"BZip2Encoder"],[3,"Deflater"],[3,"Inflater"],[3,"LzhufDecoder"],[3,"LzhufEncoder"],[3,"GZipDecoder"],[3,"GZipEncoder"],[3,"ZlibDecoder"],[3,"ZlibEncoder"]]};
initSearch(searchIndex);
