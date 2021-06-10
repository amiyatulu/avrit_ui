import React, { useState, useEffect, useContext, useCallback } from "react"
import { NearContext } from "../../commons/context/NearContext"

function FetchProductScroll(
  startPage,
  endPageLength,
  setProductsData,
  setHasMore
) {
  const { nearvar, userId } = useContext(NearContext)
  useEffect(() => {
    async function myfn() {
      try {
        let endpage
        if (startPage) {
          endpage = Math.max(0, startPage - endPageLength)
          // console.log(endpage, "endpage")

          let data = []
          for (let i = startPage; i > endpage; i--) {
            // console.log(i, "i")

            if (i <= 1) {
              setHasMore(false)
            }

            data.push(i)
          }
          const productPromises = data.map(async (x) => {
            // console.log("promisex", x)
            return nearvar.contract.get_product_js({ product_id: x.toString() })
          })

          Promise.all(productPromises).then((hash) => {
            // console.log("hash", hash)
            setProductsData((prevProducts) => {
              return [...prevProducts, ...hash]
            })
          })
        }
      } catch (e) {
        console.log(e.message)
      }
    }

    myfn()
  }, [startPage, nearvar, setProductsData, setHasMore, endPageLength])

  return
}

export default FetchProductScroll
