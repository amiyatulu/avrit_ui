const IPFS_URL = process.env.NODE_ENV === "production" ? "https://gateway.ipfs.io/ipfs/" : "https://gateway.ipfs.io/ipfs/"
const CONTRACT_NAME = process.env.NODE_ENV === "production" ? "dev-1621525348032-90260115836657" : "dev-1621525348032-90260115836657"
export { IPFS_URL, CONTRACT_NAME }