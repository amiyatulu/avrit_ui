import React, { useCallback, Component, useState } from "react"
import { useDropzone } from "react-dropzone"
import ipfs from "../../commons/ipfs"
import Resizer from 'react-image-file-resizer'
const resizeFile = (file, ext) => new Promise(resolve => {
  Resizer.imageFileResizer(file, 300, 300, ext, 100, 0,
  uri => {
    resolve(uri);
  },
  'base64'
  );
});

const base64toblog = (base64) => new Promise(resolve => {
  fetch(base64).then(res=> {
    resolve(res.blob())
  })
});

function DropProductImageUpdate(props) {
  const [ipfspath, setIpfspath] = useState(props.oldimage)
  const [loading, setLoading] = useState(false)
  async function addData(name, buffer) {
    const file = await ipfs.add({ path: name, content: buffer })
    console.log(name)
    console.log(file.cid.string)
    setIpfspath(file.cid.string)
    props.setFieldValue(props.name, file.cid.string)
    setLoading(false)
  }
  const onDrop = useCallback(async (acceptedFile) => {
    console.log(acceptedFile)
    setLoading(true)
    let ext = acceptedFile[0].path.split('.').pop().toUpperCase()
    if(ext === "JPG") {
       ext = "JPEG"
    }
    const image = await resizeFile(acceptedFile[0], ext)
    // console.log(image)
    const blob = await base64toblog(image)

    const reader = new window.FileReader()
    reader.readAsArrayBuffer(blob)
    reader.onloadend = () => {
      let buffer = Buffer(reader.result)
      console.log(buffer)
      //   setBuffer(buffer)
      addData(acceptedFile[0].path, buffer)
    }
  }, [])
  const { acceptedFiles, getRootProps, getInputProps } = useDropzone({
    onDrop,
    multiple: false,
  })

  const files = acceptedFiles.map((file) => (
    <li key={file.path}>
      {file.path}- {file.size}
      bytes
    </li>
  ))

  return (
    <section className="container">
      <div {...getRootProps({ className: "jumbotron" })}>
        <input {...getInputProps()} />
        <p>Drag 'n' drop image file, or click to select the image</p>
      </div>
      {files.length > 0 && (
        <React.Fragment>
          <div>
            <h4>Image</h4>
            <ul>{files}</ul>
          </div>
        </React.Fragment>
      )}
      {loading && (
        <React.Fragment>
          <div>
            Please wait while image loads.....<br/>
            <span className="spinner-border text-danger" role="status">
              <span className="sr-only">Loading...</span>
            </span>
          </div>
        </React.Fragment>
      )}
      {ipfspath && (
        <React.Fragment>
          <img
            src={`https://gateway.ipfs.io/ipfs/${ipfspath}`}
            alt={ipfspath}
            width="300"
            className="img-thumbnail"
          />
        </React.Fragment>
      )}
    </section>
  )
}

export default DropProductImageUpdate
