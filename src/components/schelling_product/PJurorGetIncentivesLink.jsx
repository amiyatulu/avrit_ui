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
   
   
function PJurorGetIncentivesLink(props) {
    const { nearvar, userId } = useContext(NearContext)
    const { pid } = props
    const [fetchError, setFetchError] = useState(false)
    const [canDrawIncentives, setCanDrawIncentives] = useState(false)

    useEffect(() => {
        async function fetchjurycount() {
          try {
            const canDrawIncentivesvalue = await nearvar.contract.p_if_juror_will_get_incentives({
              product_id: pid.toString(),
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
      }, [nearvar, userId, pid])
      return (
          <React.Fragment>
            { canDrawIncentives && <span className="badge badge-info">You can draw your incentives</span>
        }
        <br />
            { canDrawIncentives && <Link to={`/drawincentivesjuryproduct/${pid}/`} className="badge badge-secondary mr-3">
          Draw Jury Incentives
        </Link>}

           {/* <p> Can You Commit Vote {JSON.stringify(canDrawIncentives)}</p> */}
           {userId && <Error fetchError={fetchError} />}
           </React.Fragment>
       );
}
  
  
export default PJurorGetIncentivesLink