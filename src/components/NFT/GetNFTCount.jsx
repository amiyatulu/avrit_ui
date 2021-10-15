import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import { useParams, Link } from "react-router-dom"

function GetNFTCount(props) {
  const { nearvar, userId } = useContext(NearContext)
  const { pid } = props
  const [count, setCount] = useState(null)
  useEffect(() => {
    async function fetchPrice() {
      try {
        let data = await nearvar.contract.get_total_nft_left_for_buying_js({
          product_id: pid.toString(),
        })
        // console.log(data.toString())
        setCount(data.toString())
      } catch (e) {
        console.error(e)
        const nocounterror = e.message.includes("Nft count not set")
      }
    }

    fetchPrice()
  }, [nearvar, pid])
  return (
    <React.Fragment>
      {count && (<React.Fragment>
        <p className="badge badge-secondary mr-3">Available NFT for Buying: {count} </p>
          </React.Fragment>)}
    </React.Fragment>
  )
}

export default GetNFTCount
