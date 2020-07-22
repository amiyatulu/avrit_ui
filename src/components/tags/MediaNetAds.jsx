import React, { useEffect } from "react"

const MediaNetAds = () => {
  useEffect(() => {
    const div = document.createElement("div")
    div.id = "372802468"
    const script = document.createElement("script")
    const inlineScript = document.createTextNode(
      'try { window._mNHandle.queue.push(function (){ window._mNDetails.loadTag("372802468", "728x90", "372802468"); });}catch (error) {}'
    )
    script.appendChild(inlineScript)
    div.appendChild(script)
    document.getElementById("ads").appendChild(div)

    return () => {
        document.getElementById("ads").removeChild(div)
    }
  })

  return (
      <div id="ads"></div>
  )
}

export default MediaNetAds
