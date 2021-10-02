import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import { useParams, Link } from "react-router-dom"
import moment from "moment"
import PCommitVoteLink from "./PCommitVoteLink"
import PSelectedJuror from "./PSelectedJuror"
import PUnstakeVoteLink from "./PUnstakeVoteLInk"
import PJurorRevealLink from "./PJurorRevealLink"
import PGetTrueCount from "./PGetTrueCount"
import PGetFalseCount from "./PGetFalseCount"
import PJurorGetIncentivesLink from "./PJurorGetIncentivesLink"
import PProductIncentivesLink from "./PProductIncentivesLink"

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

// Functions available:
// get_jury_application_start_time_js + get_jury_application_phase_time => Jury application end time
// get_juror_selection_time_js + get_commit_phase_time => End of commit phase
// get_juror_selection_time_js + get_commit_phase_time + get_reveal_phase_time => End of reveal time

function PTimeConditionRender(props) {
  const { nearvar, userId } = useContext(NearContext)
  const { pid } = props
  const [errorApplication, setApplicationError] = useState(false)
  const [jurySelectionTime, setJurySelectionTime] = useState(false)
  const [time, setTime] = useState(null)
  const [endcommit, setEndCommit] = useState(null)
  const [endreveal, setEndReveal] = useState(null)
  // const { rid } = useParams()
  // console.log("rid", rid)
  useEffect(() => {
    async function fetchJuryApplicationTime() {
      let endtime = false
      try {
        let phasetimeinsec =
          await nearvar.contract.p_get_jury_application_phase_time()
        console.log("juryapplicatiophasetimeinsec", phasetimeinsec)
        let starttime =
          await nearvar.contract.p_get_jury_application_start_time_js({
            product_id: pid.toString(),
          })
        const endtime = moment.unix(
          parseInt(starttime.slice(0, 10)) + parseInt(phasetimeinsec)
        )

        return endtime
        // console.log(endtime)
        // console.log(endtime.fromNow())
      } catch (e) {
        console.log(e)
        setApplicationError(true)
      }
      return endtime
    }
    async function fetchJurySelectionTime() {
      try {
        let juryselectiontime =
          await nearvar.contract.p_get_juror_selection_time_js({
            product_id: pid.toString(),
          })

        // console.log("juryselectiontime", juryselectiontime)
        return juryselectiontime
      } catch (e) {
        console.log(e)
      }
    }

    async function fetchCommitPhaseTime() {
      try {
        let commitphasetime = await nearvar.contract.p_get_commit_phase_time()
        return commitphasetime
      } catch (e) {
        console.log(e)
      }
    }

    async function fetchRevealPhaseTime() {
      try {
        let revealphasetime = await nearvar.contract.p_get_reveal_phase_time()
        // console.log("revealphasetime", revealphasetime)
        return revealphasetime
      } catch (e) {
        console.log(e)
      }
    }

    function fetchCommitPhaseEndTime(juryselectiontime, commitphasetime) {
      // console.log(
      //   "ccjuryselectiontime",
      //   moment.unix(parseInt(juryselectiontime.slice(0, 10)))
      // )
      let endcommitdata = false
      try {
        endcommitdata = moment.unix(
          parseInt(juryselectiontime.slice(0, 10)) + parseInt(commitphasetime)
        )
      } catch (e) {
        console.log(e)
      }

      // console.log("cccomitphase", commitphasetime)
      // console.log("endcommit", endcommitdata)
      return endcommitdata
    }

    function fetchRevealPhaseEndTime(
      juryselectiontime,
      commitphasetime,
      revealphasetime
    ) {
      let endreveal = false
      try {
        endreveal = moment.unix(
          parseInt(juryselectiontime.slice(0, 10)) +
            parseInt(commitphasetime) +
            parseInt(revealphasetime)
        )
      } catch (e) {
        console.log(e)
      }
      return endreveal
    }
    async function callfetchJuryApplicationTime() {
      const endtime = await fetchJuryApplicationTime()
      console.log(endtime, "endtime")
      setTime(endtime)
    }

    async function setEndCommitEndReveal() {
      const juryselectiontime = await fetchJurySelectionTime()
      setJurySelectionTime(juryselectiontime)
      console.log(juryselectiontime, "juryselectiontime")
      const commitphasetime = await fetchCommitPhaseTime()
      // console.log(commitphasetime)
      const revealphasetime = await fetchRevealPhaseTime()
      const endcommit = fetchCommitPhaseEndTime(
        juryselectiontime,
        commitphasetime
      )
      // console.log("endcommit", endcommit)

      const endreveal = fetchRevealPhaseEndTime(
        juryselectiontime,
        commitphasetime,
        revealphasetime
      )
      // console.log("endreveal", endreveal)
      setEndCommit(endcommit)
      setEndReveal(endreveal)
    }

    callfetchJuryApplicationTime()
    setEndCommitEndReveal()
  }, [nearvar, pid])

  // if reveal time ends

  // else if commit time ends

  // else if application time ends
  console.log("endreveal", endreveal)
  if (moment().isSameOrAfter(endreveal)) {
    console.log("reveal time ends")
    return (
      <React.Fragment>
        <br />
        <span className="badge badge-info">
          Reveal Time Ended : {endreveal && endreveal.fromNow()}
        </span>
        <br />
        <PGetTrueCount pid={pid} />
        <PGetFalseCount pid={pid} />
        <br/>
        <PJurorGetIncentivesLink pid={pid} />
        <br/>
        <PProductIncentivesLink pid={pid} />
        <br/>
        
      </React.Fragment>
    )
  }

  if (moment().isSameOrAfter(endcommit)) {
    console.log("commit time ends")
    return (
      <React.Fragment>
        <br />
        {/* <span className="badge badge-info">Commit end time: {endcommit.fromNow()}</span> <br /> */}
        <span className="badge badge-info">
          End reveal time : {endreveal && endreveal.fromNow()}
        </span>
        <br />
        <PJurorRevealLink pid={pid} /> <br />
        <PGetTrueCount pid={pid} />
        <PGetFalseCount pid={pid} />
      </React.Fragment>
    )
  }

  // console.log(jurySelectionTime, "jury selection time");
  // console.log(time, "time")
  // console.log(endcommit, "commitendtime")

  let juryselectiontime_slice
  if (jurySelectionTime) {
    juryselectiontime_slice = moment.unix(jurySelectionTime.slice(0, 10))
  }
  let unstake
  if (moment().isSameOrAfter(moment(juryselectiontime_slice).add(1, "hours")) &&  jurySelectionTime !== undefined) {
    unstake = <PUnstakeVoteLink pid={pid} />
  }

  if (
    moment().isSameOrAfter(time) &&
    moment().isSameOrAfter(juryselectiontime_slice) &&
    jurySelectionTime !== undefined
  ) {
    return (
      <React.Fragment>
        <br />
        <span className="badge badge-info">
          Commit end time: {endcommit && endcommit.fromNow()}
        </span>
        <br />
        {/* To do: If already commited don't render commit vote */}
        <PSelectedJuror pid={pid} />

        {unstake}
        <PCommitVoteLink pid={pid} />
      </React.Fragment>
    )
  }

  if (jurySelectionTime === undefined && moment().isSameOrAfter(time)) {
    return (
      <React.Fragment>
        <PSelectedJuror pid={pid} />
        <Link to={`/drawjuryforproduct/${pid}`} className="badge badge-secondary mr-3">
          Draw Juror
        </Link>
      </React.Fragment>
    )
  }

  if (moment().isSameOrBefore(time)) {
    return (
      <React.Fragment>
        <br />
        <span className="badge badge-info">
          Jury application end time: {time.fromNow()}
        </span>
        <br />
      </React.Fragment>
    )
  }

  return <React.Fragment></React.Fragment>
}

export default PTimeConditionRender
