import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import "./ProgressBar.css"
import { BigNumber } from "bignumber.js";

function ProgressBar() {
  const { nearvar } = useContext(NearContext)
  const [start, setStart] = useState(0)
  const [end, setEnd] = useState(0)
  let pw = BigNumber(10).pow(18)
  useEffect(() => {
    async function getPriceCount() {
      try {
        const startcount = await nearvar.contract.get_token_sold()
        setStart(startcount)

        // new start =  startcount - begin token sold

        const tokenavailable =
          await nearvar.contract.get_phase_available_tokens()
        
        setEnd(tokenavailable)
        // new end = tokenavailable - begin token sold 
      } catch (e) {
        console.error(e.message)
      }
    }
    getPriceCount()
  }, [nearvar])
  return (
    <React.Fragment>
      <div className="progress progresscss">
        <div
          className="progress-bar bg-success progress-bar-striped"
          role="progressbar"
          aria-valuenow={start}
          aria-valuemin="0"
          aria-valuemax={end}
        ></div> 
      </div>
      <br/><br/>
      <div className="alert bg-success text-white">Total Avrit tokens on sale: {BigNumber(end).div(pw).toFixed()}</div>
      <div className="alert bg-success text-white">Sold Avrit Tokens: {BigNumber(start).div(pw).toFixed()}</div>

    </React.Fragment>
  )
}

export default ProgressBar
