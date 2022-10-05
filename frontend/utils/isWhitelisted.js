import { hashToKeccak256 } from './keccak256'

const { MerkleTree } = require('merkletreejs')
const SHA256 = require('crypto-js/sha256')

export const isWhitelisted = (target) => {
  const leaves = [
    '3ac225168df54212a25c1c01fd35bebfea408fdac2e31ddd6f80a4bbf9a5f1cb',
    '62c4ac5687f978c211efcb9852060ca0d29e10ef0ce82f45c0ce74fdfaf3966c',
  ].map((x) => SHA256(x))
  const tree = new MerkleTree(leaves, SHA256)
  const root = tree.getRoot().toString('hex')

  const leaf = SHA256(hashToKeccak256(target))
  const proof = tree.getProof(leaf)

  //   console.log(hashToKeccak256('firas.testnet'))
  //   console.log('This is the root', root)

  return target === 'a' ? false : tree.verify(proof, leaf, root)
}
