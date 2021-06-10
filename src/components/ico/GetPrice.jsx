import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
   
   
function GetPrice(numberOfTokens) {
    const { nearvar} = useContext(NearContext)
    const [count, setCount] = useState(0)
    useEffect(() => {
      async function getPriceCount() {
        try {
          const countvalue = await nearvar.contract.required_deposit({
            number_of_tokens: numberOfTokens.toString(),
          })
          setCount(countvalue)
        } catch (e) {
          console.error(e.message)
        }
      }
      getPriceCount()
    }, [nearvar, numberOfTokens])
      return {count};
}
  
  
export default GetPrice