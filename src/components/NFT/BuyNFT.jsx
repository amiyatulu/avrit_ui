import React, { useState, useContext } from "react"
import * as Yup from "yup"
import { Formik, Form, Field } from "formik"
import { useHistory, useParams, useLocation } from "react-router-dom"
import ipfs from "../../commons/ipfs"
import { NearContext } from "../../commons/context/NearContext"
import { FocusError, SubmittingWheel } from "../../commons/FocusWheel"
import GetNFTPriceBuy from "./GetNFTPriceBuy"
import { BigNumber } from "bignumber.js"

function BuyNFT(props) {
  // const [count, setCount] = useState(0);
  const { pid } = useParams()
  let location = useLocation()
  let history = useHistory()
  let { nearvar } = useContext(NearContext)
  const [errorThrow, setErrorThrow] = useState(false)
  let price  = GetNFTPriceBuy(pid)
  let pw = BigNumber(10).pow(18)
  console.log("price", price)
  return (
    <React.Fragment>
      <div className="container">
        <Formik
          initialValues={{}}
          validationSchema={Yup.object().shape({})}
          onSubmit={async (values, actions) => {
            try {

              //   values.countvariable = count
              await nearvar.contract.buy_nft({
                args:{
                  token_id: pid.toString(),
                  amount: price,
                },
                callbackUrl: `${window.location.origin.toString()}/#/viewnft`,
                amount: "7100000000000000000000",
                gas: 95000000000000

              })
              actions.setSubmitting(false)
              // console.log(data)
              // history.push(`/viewnft`)
              // history.goBack()
            } catch (e) {
              console.error(e)
              setErrorThrow(e.message)
            }
          }}
        >
          {({
            handleSubmit,
            handleBlur,
            handleChange,
            errors,
            touched,
            isValid,
            isSubmitting,
            values,
            setFieldValue,
            validateForm,
          }) => (
            <Form onSubmit={handleSubmit}>
              {errorThrow && <p>{errorThrow}</p>}
              {price && (
                <React.Fragment>
                  <p className="badge badge-secondary mr-3">
                    NFT Price: {BigNumber(price).div(pw).toFixed()} Avrit
                  </p>
                </React.Fragment>
              )}
              <div className="text-center">
                <button
                  type="submit"
                  className="btn btn-primary"
                  disabled={isSubmitting}
                >
                  Buy NFT
                </button>
              </div>
              <SubmittingWheel isSubmitting={isSubmitting} />
              <FocusError />
            </Form>
          )}
        </Formik>
      </div>
    </React.Fragment>
  )
}

export default BuyNFT
