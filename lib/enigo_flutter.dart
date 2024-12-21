library enigo_flutter;

import 'src/rust/api/enigo.dart';
export 'src/rust/api/enigo.dart';
export 'src/rust/frb_generated.dart' show RustLib;

final enigo = Enigo.preset();
