import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
   
   
function ViewNFT() {
const { nearvar, userId } = useContext(NearContext)
  useEffect(() => {
    async function getNFT() {
        try {
            let data = await nearvar.contract.last_ten_tokens_for_owner({
                user_id: userId.toString(),
            })
            // console.log(data.toString())
            console.log(data)
          } catch (e) {
            console.error(e)
            const nopriceerror = e.message.includes("Price not set")
          }

    }
  
    getNFT()
  }, [nearvar, userId])
      return (
          <React.Fragment>
     
           </React.Fragment>
       );
}
  
  
export default ViewNFT