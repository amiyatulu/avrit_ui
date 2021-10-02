import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import { useParams, Link } from "react-router-dom"
import { BigNumber } from "bignumber.js"

function GetNFTPrice(props) {
  const { nearvar, userId } = useContext(NearContext)
  const { pid } = props
  const [price, setPrice] = useState(null)
  let pw = BigNumber(10).pow(18)
  useEffect(() => {
    async function fetchPrice() {
      try {
        let data = await nearvar.contract.get_nft_price_js({
          product_id: pid.toString(),
        })
        // console.log(data.toString())
        setPrice(data.toString())
      } catch (e) {
        console.error(e)
        const nopriceerror = e.message.includes("Price not set")
      }
    }

    fetchPrice()
  }, [nearvar, pid])
  return (
    <React.Fragment>
      {price && (<React.Fragment>
        <p className="badge badge-secondary mr-3">NFT Price: {BigNumber(price).div(pw).toFixed()} Avrit </p>
          </React.Fragment>)}
    </React.Fragment>
  )
}

export default GetNFTPrice
