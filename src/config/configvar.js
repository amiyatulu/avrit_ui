const IPFS_URL = process.env.NODE_ENV === "production" ? "https://gateway.ipfs.io/ipfs/" : "https://gateway.ipfs.io/ipfs/"
const CONTRACT_NAME = process.env.NODE_ENV === "production" ? "dev-1623318996331-11938261360938" : "dev-1623318996331-11938261360938"
export { IPFS_URL, CONTRACT_NAME }