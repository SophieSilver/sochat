import 'package:client/src/rust/api/service.dart';

class RustService {
  static final Service _instance = Service();
  
  static Service get instance => _instance; 
}