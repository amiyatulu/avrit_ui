import React, { useCallback, useState, useEffect} from "react"
import { useDropzone } from "react-dropzone"
import ipfs from "../../commons/ipfs"

function DropProductPDFs(props) {
  const [ipfspdfpaths, setIpfspdfpaths] = useState([])
  const [loading, setLoading] = useState(false)
  
  useEffect(() => {
    props.setFieldValue(props.name, ipfspdfpaths.join(","))
    // console.log( ipfspdfpaths.join(","), "ipfspdfpaths")
  }, [ipfspdfpaths])

  const addDataFiles = (bufferResults) => {
    const addDataPromises = bufferResults.map((res) => {
      return ipfs.add({ path: res[0], content: res[1] })
    })

    Promise.all(addDataPromises).then((results) => {
      //   console.log(results)
      let cidmap = []
      results.map((cidstring) => {
        cidmap.push(cidstring.cid.string)
      })
      //   console.log(cidmap)
      setIpfspdfpaths(cidmap)
      setLoading(false)
    })
  }

  const uploadFiles = () => {
    // console.log(acceptedFiles)
    setLoading(true)
    const readerPromises = acceptedFiles.map((file) => {
      return new Promise((resolve) => {
        const reader = new window.FileReader()
        reader.readAsArrayBuffer(file)
        reader.onloadend = () => {
          // console.log(file);
          let buffer = Buffer(reader.result) // Buffer(reader.result)
          resolve([file.name, buffer])
        }
      })
    })

    Promise.all(readerPromises).then((results) => {
      // console.log(results);
      addDataFiles(results)
      // set loading to false;
    })
  }
  const { acceptedFiles, getRootProps, getInputProps } = useDropzone()

  const files = acceptedFiles.map((file) => (
    <li key={file.path}>
      {file.path}- {file.size}
      bytes
    </li>
  ))

  const links = ipfspdfpaths.map((path, index) => (
    <React.Fragment key={index}>
      <div>
      <br/>
      <a target='_blank'
        href={`https://gateway.ipfs.io/ipfs/${path}`}
      >{`https://gateway.ipfs.io/ipfs/${path}`}</a>
      </div>
    </React.Fragment>
  ))

  return (
    <section className="container">
      <div {...getRootProps({ className: "jumbotron" })}>
        <input {...getInputProps()} />
        <p>Drag 'n' drop pdfs, or click to select the pdfs</p>
      </div>
      {files.length > 0 && (
        <React.Fragment>
          <div>
            <h4>PDFs</h4>
            <ul>{files}</ul>
          </div>
          <button
            onClick={() => {
              uploadFiles()
            }}
            disabled={loading}
            className="btn btn-warning"
          >
            Upload
          </button>
        </React.Fragment>
      )}
      {loading && (
        <React.Fragment>
          <div>
            Please wait while image loads.....
            <br />
            <span className="spinner-border text-danger" role="status">
              <span className="sr-only">Loading...</span>
            </span>
          </div>
        </React.Fragment>
      )}
      {ipfspdfpaths && <React.Fragment>{links}</React.Fragment>}
    </section>
  )
}

export default DropProductPDFs
