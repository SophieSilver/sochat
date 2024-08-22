import 'package:client/src/rust/api/service.dart';

class RustService {
  static late RustServiceInstance _instance;
  static late Stream<void> _messageNotificationStream;

  static RustServiceInstance get instance => _instance;

  static Future<void> init() async {
    _instance = await RustServiceInstance.internalInit(); 
    _messageNotificationStream = _instance.internalInitMessageStream();
  }

}
