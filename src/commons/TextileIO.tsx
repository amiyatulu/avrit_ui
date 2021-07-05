import {
  Buckets,
  Client,
  ThreadID,
  PrivateKey,
  Where,
  Identity,
  KeyInfo,
} from "@textile/hub"

async function getIdentity() {
  try {
    var storedIdent = localStorage.getItem("identity")
    if (storedIdent === null) {
      throw new Error("No identity")
    }
    const restored = PrivateKey.fromString(storedIdent)
    return restored
  } catch (e) {
    /**
     * If any error, create a new identity.
     */
    try {
      const identity = PrivateKey.fromRandom()
      const identityString = identity.toString()
      localStorage.setItem("identity", identityString)
      return identity
    } catch (err) {
      return err.message
    }
  }
}
async function getBucketKey(identity: Identity, keyInfo: KeyInfo) {
  const buckets = await Buckets.withKeyInfo(keyInfo)
  // Authorize the user and your insecure keys with getToken
  await buckets.getToken(identity)

  const buck = await buckets.getOrCreate("io.textile.dropzone")
  if (!buck.root) {
    throw new Error("Failed to open bucket")
  }
  return { buckets: buckets, bucketKey: buck.root.key }
}


async function insertFile (buckets: Buckets, bucketKey: string, file: string, path: string) {
  const buf = Buffer.from(file)
  let raw = await buckets.pushPath(bucketKey, path, buf)
  return raw
}
// async function add(
//   buckets: Buckets,
//   bucketKey: string,
//   content: File,
//   filename: string
// ) {
//   const upload = {
//     path: filename,
//     content,
//   }
//   const raw = buckets.pushPath(bucketKey, filename, upload)
//   return raw
// }
async function ipfs(content: string, filename: string) {
  const keyInfo: KeyInfo = {
    key: process.env.REACT_APP_TEXTILEIO_KEY as string,
    secret: process.env.REACT_APP_TEXTILEIO_SECRET as string,
  }

  const identity = await getIdentity()

  const { bucketKey, buckets } = await getBucketKey(identity, keyInfo)

  console.log(buckets)
  console.log(bucketKey)
  // const content = JSON.stringify({ "python3": "Ubuntu" });
  // const filename = "file.json"
  const data = await insertFile(buckets, bucketKey, content, filename)
  return data.path
}
export default ipfs
