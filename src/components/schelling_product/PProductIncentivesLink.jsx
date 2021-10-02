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
   
   
function PProductIncentivesLink(props) {
    const { nearvar, userId } = useContext(NearContext)
    const { pid } = props
    const [fetchError, setFetchError] = useState(false)
    const [canDrawIncentives, setCanDrawIncentives] = useState(false)

    useEffect(() => {
        async function fetchjurycount() {
          try {
            const canDrawIncentivesvalue = await nearvar.contract.p_if_product_get_incentives_bool({
              product_id: pid.toString()
            })
            console.log("canDrawIncentivesValue", canDrawIncentivesvalue)
            setCanDrawIncentives(canDrawIncentivesvalue)
          } catch (e) {
            console.error(e.message)
            setFetchError(e.message)
          }
        }
        fetchjurycount()
      }, [nearvar, pid])
      return (
          <React.Fragment>
            { canDrawIncentives && <span className="badge badge-info">Product owner can draw incentives</span>
        }
        <br />
            { canDrawIncentives && <Link to={`/productdrawincentives/${pid}/`} className="badge badge-secondary mr-3">
          Draw Incentives for Product
        </Link>}

           {/* <p> Can You Commit Vote {JSON.stringify(canDrawIncentives)}</p> */}
           {userId && <Error fetchError={fetchError} />}
           </React.Fragment>
       );
}
  
  
export default PProductIncentivesLink