import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import { useParams, Link } from "react-router-dom"
import moment from "moment"

function Loading(props) {
  const { error } = props
  if (error) {
    return <React.Fragment></React.Fragment>
  }
  return (
    <p className="container">
      Loading
      <span role="img" aria-label="loading">
        ⌛
      </span>
    </p>
  )
}

function JuryApplyTime(props) {
  const { nearvar, userId } = useContext(NearContext)
  const { rid } = props
  const [time, setTime] = useState(null)
  const [error, setError] = useState(false)
  // const { rid } = useParams()
  console.log("rid", rid)
  useEffect(() => {
    async function fetchJuryApplicationTime() {
      try {
        let phasetimeinsec = await nearvar.contract.get_jury_application_phase_time()
        console.log(phasetimeinsec)
        let starttime = await nearvar.contract.get_jury_application_start_time_js(
          {
            review_id: parseInt(rid),
          }
        )
        console.log(starttime)
        let timeleft = moment
          .unix(starttime.slice(0, 10))
          .add(phasetimeinsec, "seconds")
        console.log(timeleft.fromNow())
        setTime(timeleft)
      } catch (e) {
        console.log(e)
        setError(true)
      }
    }

    fetchJuryApplicationTime()
  }, [nearvar, rid])
  return (
    <React.Fragment>
      {time ? (
        <React.Fragment>
          <span>Jury application end time: {time.fromNow()}</span>
        </React.Fragment>
      ) : (
        <Loading error={error} />
      )}
    </React.Fragment>
  )
}

export default JuryApplyTime
