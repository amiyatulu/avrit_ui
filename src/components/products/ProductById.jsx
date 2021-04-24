import React, { useState, useContext, useEffect } from "react"
import { NearContext } from "../../commons/context/NearContext"
import { useParams, Link } from "react-router-dom"
import axios from "axios"
import styles from "../profile/ViewProfile.module.css"
import { IPFS_URL } from "../../config/configvar"
import GetReviews from "../reviews/GetReviews"
import longword from "./LongWords.module.css"
import Linkify from "react-linkify"
import TagsStyle from "./TagsStyle"
import ProductTypeStyle from "./ProductTypeStyle"

function ProductById() {
  const { nearvar, userId } = useContext(NearContext)
  const { id } = useParams()
  const [ipfsData, setProductData] = useState(null)
  const [productType, setProductType] = useState(null)
  const [productUserId, setProductUserId] = useState(null)

  useEffect(() => {
    async function fetchProduct() {
      try {
        let data = await nearvar.contract.get_product({
          product_id: parseInt(id),
        })
        console.log("product data", data)
        setProductType(data.product_type)
        setProductUserId(data.user_id)
        const result = await axios(`${IPFS_URL}${data.product_details_hash}`)
        console.log(result.data)
        setProductData(result.data)
      } catch (e) {
        console.error(e)
      }
    }

    fetchProduct()
  }, [])
  if (!ipfsData) {
    return (
      <React.Fragment>
        <div className="container">
          <div className="d-flex justify-content-center">
            <div className="spinner-grow text-warning" role="status">
              <span className="sr-only">Loading...</span>
            </div>
          </div>
        </div>
      </React.Fragment>
    )
  }

  return (
    <React.Fragment>
      {ipfsData && (
        <div className="container">
          <div className="jumbotron">
            <h3 className="display-4">{ipfsData.headline}</h3>
            {ipfsData.productimage && (
              <React.Fragment>
                <a
                  target="_blank"
                  href={`https://gateway.ipfs.io/ipfs/${ipfsData.productimage}`}
                >
                  <img
                    src={`https://gateway.ipfs.io/ipfs/${ipfsData.productimage}`}
                    alt="productimage"
                    width="300"
                    className="img-thumbnail"
                  />
                </a>
                <br />
                <br />
              </React.Fragment>
            )}

            <h5>Details:</h5>
            <p className={`${longword.linebreaks} ${longword.wraplongworld}`}>
              <Linkify
                componentDecorator={(decoratedHref, decoratedText, key) => (
                  <a target="blank" href={decoratedHref} key={key}>
                    {decoratedText}
                  </a>
                )}
              >
                {ipfsData.details}
              </Linkify>
            </p>
            <h5>PDFs:</h5>
            <div>
              {ipfsData.pdfs && (
                <React.Fragment>
                  {ipfsData.pdfs.split(",").map((path, index) => (
                    <React.Fragment key={index}>
                      <div>
                        <a
                          className={longword.wraplongworld}
                          target="_blank"
                          href={`https://gateway.ipfs.io/ipfs/${path}`}
                        >{`https://gateway.ipfs.io/ipfs/${path}`}</a>
                      </div>
                    </React.Fragment>
                  ))}
                </React.Fragment>
              )}
            </div>
            <br />
            <h5>Specialization:</h5>
            <p>
              <TagsStyle tags={ipfsData.specialization} />
            </p>
            <h5>Audience:</h5>
            <p>
              <TagsStyle tags={ipfsData.audience} />
            </p>
            <h5>Product Type:</h5>
            <p>
              <ProductTypeStyle type={productType} />
            </p>
            <Link
              to={`/createreview/${id}`}
              className="badge badge-secondary mr-3"
            >
              Create Review
            </Link>
            {productUserId === userId && (
              <Link
                to={`/updateproduct/${id}`}
                className="badge badge-secondary mr-3"
              >
                Update Product
              </Link>
            )}
          </div>
          <div>
            <GetReviews pid={id} />
          </div>
        </div>
      )}
    </React.Fragment>
  )
}

export default ProductById
