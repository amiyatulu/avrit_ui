import React, { useState, useEffect, useContext, useCallback } from "react"
import InfiniteScroll from "react-infinite-scroll-component"
import FetchProductScroll from "./FetchProductScroll"
import { NearContext } from "../../commons/context/NearContext"
import ProductDetailsHome from "./ProductDetailsHome"
import "./FetchProductsPage.css"

function FetchProductsPage() {
  const { nearvar, userId } = useContext(NearContext)
  const [productsData, setProductsData] = useState([])
  const [startPage, setStartPage] = useState(null)
  const endPageLength = 10
  const [hasMore, setHasMore] = useState(true)
  // console.log(hasMore, "hasmore")

  FetchProductScroll(startPage, endPageLength, setProductsData, setHasMore)
  useEffect(() => {
    // console.log("hello use effect")
    async function myfn() {
      try {
        const countvalue = await nearvar.contract.get_final_product_id({})
        // console.log(countvalue)
        setStartPage(parseInt(countvalue))
      } catch (e) {
        console.error(e.message)
      }
    }

    myfn()
  }, [nearvar])

  function fetchMoreData() {
    // a async api call like which sends
    // more records in 1.5 secs
    setTimeout(() => {
      setStartPage((prevCount) => {
        return prevCount - endPageLength
      })
      // console.log(startPage, "startpage")
      // console.log("in timeout")
    }, 1500)
  }
  const rows = productsData.reduce(function (rows, key, index) {
    return (
      (index % 3 === 0 ? rows.push([key]) : rows[rows.length - 1].push(key)) &&
      rows
    )
  }, [])

  return (
    <React.Fragment>
      <div className="container myscrolldiv">
        <InfiniteScroll
          dataLength={productsData.length}
          next={fetchMoreData}
          hasMore={hasMore}
          loader={<h4>Loading...</h4>}
        >
          {rows &&
            rows.map((row, index) => (
              <React.Fragment key={index}>
                <div className="row productrow">
                  {row.map((data) => (
                    <React.Fragment>
                      {data.product_expired === false && (
                        <div
                          className="col-md-4 productcol"
                          key={data.product_id}
                        >
                          <React.Fragment>
                            {/* {JSON.stringify(data)} */}
                            <ProductDetailsHome
                              ipfshash={data.product_details_hash}
                              id={data.product_id}
                            />
                          </React.Fragment>
                        </div>
                      )}
                    </React.Fragment>
                  ))}
                </div>
              </React.Fragment>
            ))}
          {/* {productsData &&
          productsData.map((data) => (
            <React.Fragment key={data.product_id}>
              <ProductDetails
                ipfshash={data.product_details_hash}
                id={data.product_id}
              />
            </React.Fragment>
          ))} */}
        </InfiniteScroll>
      </div>
    </React.Fragment>
  )
}

export default FetchProductsPage
