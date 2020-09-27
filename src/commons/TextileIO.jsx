import { Buckets, Client, ThreadID, PrivateKey, Where } from '@textile/hub';


async function getIdentity() {
    try {
        var storedIdent = localStorage.getItem("identity")
        if (storedIdent === null) {
            throw new Error('No identity')
        }
        const restored = PrivateKey.fromString(storedIdent)
        return restored
    }
    catch (e) {
        /**
         * If any error, create a new identity.
         */
        try {
            const identity = await PrivateKey.fromRandom()
            const identityString = identity.toString()
            localStorage.setItem("identity", identityString)
            return identity
        } catch (err) {
            return err.message
        }
    }
}

async function getBucketKey(identity, keyInfo, keyInfoOptions) {
    if (!identity) {
        throw new Error('Identity not set')
    }
    const buckets = await Buckets.withKeyInfo(keyInfo, keyInfoOptions)
    // Authorize the user and your insecure keys with getToken
    await buckets.getToken(identity)

    const buck = await buckets.getOrCreate('io.textile.dropzone')
    if (!buck.root) {
        throw new Error('Failed to open bucket')
    }
    return { buckets: buckets, bucketKey: buck.root.key };
}

async function add(buckets, bucketKey, content, filename) {
    const upload = {
        path: filename,
        content
    }
    const raw = buckets.pushPath(bucketKey, filename, upload)
    return raw
}
async function Ipfsadd(content, filename) {
    const keyInfo = {
        key: "bj6pe326abw3utf6dwmbdxuazta",
    }

    const keyInfoOptions = {
        debug: false
    }

    const identity = await getIdentity();

    const { bucketKey, buckets } = await getBucketKey(identity, keyInfo, keyInfoOptions)

    console.log(buckets);
    console.log(bucketKey);
    // const content = JSON.stringify({ "python3": "Ubuntu" });
    // const filename = "file.json"
    const data = await add(buckets, bucketKey, content, filename);
    return data
}
export default Ipfsadd