// In production, do not imprt style.scss, as it is provided by devand-web
// import './static/style.scss';

import("./pkg").then(module => {
  module.run_app();
});
