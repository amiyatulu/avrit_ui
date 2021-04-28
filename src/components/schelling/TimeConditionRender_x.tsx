import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import { useParams, Link } from "react-router-dom"
import moment from "moment"

function Loading(props: { error: string }) {
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

// Functions available:
// get_jury_application_start_time_js + get_jury_application_phase_time => Jury application end time
// get_juror_selection_time_js + get_commit_phase_time => End of commit phase
// get_juror_selection_time_js + get_commit_phase_time + get_reveal_phase_time => End of reveal time

function TimeConditionRender(props: {
  nearvar: any
  userId: string
  rid: string
}) {
  const { nearvar, userId } = useContext(NearContext)
  const { rid } = props
  const [juryApplicationEndTime, setJuryApplicationEndTime] = useState<any>(0)

  useEffect(() => {
    async function fetchJuryApplicationTime(): Promise<any>{
      try {
        let phasetimeinsec = await nearvar.contract.get_jury_application_phase_time()
        let jury_application_start_time = await nearvar.contract.get_jury_application_start_time_js(
          {
            review_id: rid.toString(),
          }
        )
         const endtime = moment.unix(
          parseInt(jury_application_start_time.slice(0, 10)) + parseInt(phasetimeinsec)
        )
        return endtime
        // console.log("jury_application_start_time", jury_application_start_time)
      } catch (e) {
        console.log(e)
        return 0
      }
    }

    async function fetchJurySelectionTime() {
      try {
        let juryselectiontime = await nearvar.contract.get_juror_selection_time_js(
          {
            review_id: rid.toString(),
          }
        )

        // console.log("juryselectiontime", juryselectiontime)
        return juryselectiontime
      } catch (e) {
        console.log(e)
      }
    }
    
    async function callfetchJuryApplicationTime() {
      let endtime = await fetchJuryApplicationTime();
      // console.log("endtime", endtime)
      setJuryApplicationEndTime(endtime)
    }

    // Final calls
    callfetchJuryApplicationTime()
   
  })

  if (moment().isSameOrBefore(juryApplicationEndTime)) {
    return (
      <React.Fragment>
        <br />
        <span>Jury application end time: {juryApplicationEndTime.fromNow()}</span> <br />
      </React.Fragment>
    )
  }

  return <React.Fragment></React.Fragment>
}

export default TimeConditionRender
