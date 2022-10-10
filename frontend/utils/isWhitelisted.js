import { councilMembers } from '../data/Whitelist'
import { hashToKeccak256 } from './keccak256'

const { MerkleTree } = require('merkletreejs')
const SHA256 = require('crypto-js/sha256')

export const isWhitelisted = (target) => {
  const leaves = councilMembers.map((x) => SHA256(x))
  const tree = new MerkleTree(leaves, SHA256)
  const root = tree.getRoot().toString('hex')

  const leaf = SHA256(hashToKeccak256(target))
  const proof = tree.getProof(leaf)

  //   console.log(hashToKeccak256('firas.testnet'))
  //   console.log('This is the root', root)

  return target === 'a' ? false : tree.verify(proof, leaf, root)
}
