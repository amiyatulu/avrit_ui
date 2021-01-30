import React, { useCallback, Component, useState } from "react"
import { useDropzone } from "react-dropzone"
import ipfs from "../../commons/ipfs"

function DropProductImage(props) {
  //   const [buffer, setBuffer] = useState(null)
  async function addData(name, buffer) {
    const file = await ipfs.add({ path: name, content: buffer })
    console.log(name)
    console.log(file.cid.string)
  }
  const onDrop = useCallback((acceptedFile) => {
    console.log(acceptedFile)

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
      <div {...getRootProps({ className: "dropzone" })}>
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
    </section>
  )
}

export default DropProductImage
