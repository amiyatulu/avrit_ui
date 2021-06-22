import React, { useState } from "react"
import "./PulseEffectTitle.css"

function PulseEffectTitle() {
  const [count, setCount] = useState(0)
  return (
    <React.Fragment>
        <br/>

      <h1 className="text-center heading">Crowdsale</h1>
      <div className="pulsebody">
      <div className="blobs-container">
        <div className="blob white"></div>
        <div className="blob red"></div>
        <div className="blob orange"></div>
        <div className="blob yellow"></div>
        <div className="blob blue"></div>
        <div className="blob green"></div>
        <div className="blob purple"></div><div className="blob"></div>
      </div>
      </div>
      <br/>
      <br/>
    </React.Fragment>
  )
}

export default PulseEffectTitle
