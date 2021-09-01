import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
   
   
function GetNFTPriceBuy(pid) {
    const { nearvar, userId } = useContext(NearContext)
    const [price, setPrice] = useState(null)
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
      return price;
}
  
  
export default GetNFTPriceBuy