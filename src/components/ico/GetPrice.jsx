import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
   
   
function GetPrice(numberOfTokens) {
    const { nearvar} = useContext(NearContext)
    const [count, setCount] = useState(0)
    // console.log(numberOfTokens.toLocaleString('fullwide', {useGrouping:false}), "number of tokens in getprice")
    console.log(numberOfTokens.toFixed(), "number of tokens")
    useEffect(() => {
      async function getPriceCount() {
        try {
          const countvalue = await nearvar.contract.required_deposit({
            number_of_tokens: numberOfTokens.toFixed(),
            // numberOfTokens.toLocaleString('fullwide', {useGrouping:false})
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