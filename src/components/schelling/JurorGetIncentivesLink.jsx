import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import { Link } from "react-router-dom"

function Error(props) {
  const { fetchError } = props
  if (fetchError) {
    return <span className="container">{fetchError}</span>
  }
  return <React.Fragment></React.Fragment>
}
   
   
function JurorGetIncentivesLink(props) {
    const { nearvar, userId } = useContext(NearContext)
    const { rid } = props
    const [fetchError, setFetchError] = useState(false)
    const [canDrawIncentives, setCanDrawIncentives] = useState(false)

    useEffect(() => {
        async function fetchjurycount() {
          try {
            const canDrawIncentivesvalue = await nearvar.contract.if_juror_will_get_incentives({
              review_id: rid.toString(),
              user_id: userId.toString(),
            })
            console.log("canDrawIncentivesValue", canDrawIncentivesvalue)
            setCanDrawIncentives(canDrawIncentivesvalue)
          } catch (e) {
            console.error(e.message)
            setFetchError(e.message)
          }
        }
        fetchjurycount()
      }, [nearvar, userId, rid])
      return (
          <React.Fragment>
            { canDrawIncentives && <Link to={`/drawjurorincentives/${rid}/`} className="badge badge-secondary mr-3">
          Draw Incentives
        </Link>}
           {/* <p> Can You Commit Vote {JSON.stringify(canDrawIncentives)}</p> */}
           {userId && <Error fetchError={fetchError} />}
           </React.Fragment>
       );
}
  
  
export default JurorGetIncentivesLink