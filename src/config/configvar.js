const IPFS_URL = process.env.NODE_ENV === "production" ? "https://gateway.ipfs.io/ipfs/" : "https://gateway.ipfs.io/ipfs/"
const CONTRACT_NAME = process.env.NODE_ENV === "production" ? "dev-1619608560359-4603471" : "dev-1619608560359-4603471"
export { IPFS_URL, CONTRACT_NAME }