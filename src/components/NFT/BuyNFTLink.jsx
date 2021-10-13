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

   
function BuyNFTLink(props) {
    const { nearvar, userId } = useContext(NearContext)
    const { pid } = props
    const [fetchError, setFetchError] = useState(false)
    const [canBuyNFT, setCanBuyIncentives] = useState(false)

    useEffect(() => {
        async function fetchcanbuynft() {
          try {
            const canbuynft = await nearvar.contract.display_buy_nft({
              product_id: pid.toString(),
            })
            // console.log("canDrawIncentivesValue", canbuynft)
            setCanBuyIncentives(canbuynft)
          } catch (e) {
            console.error(e.message)
            setFetchError(e.message)
          }
        }
        fetchcanbuynft()
      }, [nearvar, pid])
      return (
        <React.Fragment>
    <br />
        { canBuyNFT && <Link to={`/buynft/${pid}`} className="badge badge-secondary mr-3">
              Buy NFT
            </Link>}

       {/* <p> Can You Commit Vote {JSON.stringify(canDrawIncentives)}</p> */}
       {userId && <Error fetchError={fetchError} />}
       </React.Fragment>
       );
}
  
  
export default BuyNFTLink

