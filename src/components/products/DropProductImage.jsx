import React, { useCallback, Component, useState } from "react"
import { useDropzone } from "react-dropzone"
import ipfs from "../../commons/ipfs"

function DropProductImage(props) {
  const [ipfspath, setIpfspath] = useState(null)
  const [loading, setLoading] = useState(false)
  async function addData(name, buffer) {
    const file = await ipfs.add({ path: name, content: buffer })
    console.log(name)
    console.log(file.cid.string)
    setIpfspath(file.cid.string)
    props.setFieldValue(props.name, file.cid.string)
    setLoading(false)
  }
  const onDrop = useCallback((acceptedFile) => {
    console.log(acceptedFile)
    setLoading(true)

    const reader = new window.FileReader()
    reader.readAsArrayBuffer(acceptedFile[0])
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

export default DropProductImage
