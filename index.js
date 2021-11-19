const { resize: _resize } = require('./js-binding')

module.exports.resize = function resize(input, outputWidth, outputHeight) {
  return _resize(input, outputWidth, outputHeight)
}
