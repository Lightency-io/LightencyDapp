const keccak256 = require('keccak256')

export const hashToKeccak256 = (string) => {
  return keccak256(string).toString('hex')
}
