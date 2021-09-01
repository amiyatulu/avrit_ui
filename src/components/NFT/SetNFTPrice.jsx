import React, { useState, useContext } from "react"
import * as Yup from "yup"
import { Formik, Form, Field } from "formik"
import { useHistory, useParams } from "react-router-dom"
import ipfs from "../../commons/ipfs"
import { NearContext } from "../../commons/context/NearContext"
import { FocusError, SubmittingWheel } from "../../commons/FocusWheel"
import { BigNumber } from "bignumber.js"

function SetNFTPrice(props) {
  // const [count, setCount] = useState(0);
  const { pid } = useParams()
  let history = useHistory()
  let { nearvar } = useContext(NearContext)
  const [errorThrow, setErrorThrow] = useState(false)
  let pw = BigNumber(10).pow(24)

  return (
    <React.Fragment>
      <div className="container">
        <br/>
        <br/>
        <Formik
          initialValues={{
            price: "",
            token_count: "",
          }}
          validationSchema={Yup.object().shape({
            price: Yup.string().required("Price is required"),
            token_count: Yup.string().required("NFT count is required"),
          })}
          onSubmit={async (values, actions) => {
            try {
              //   values.countvariable = count
              let attotokens = BigNumber(values.price).times(pw)
              console.log(attotokens.toFixed())
              await nearvar.contract.setup_nft_price_and_token_count({
                product_id: pid.toString(),
                price: attotokens.toFixed(),
                token_count: values.token_count.toString(),
              })
              actions.setSubmitting(false)
              // console.log(data)
               history.push(`/product/${pid}`)
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

              <div className="form-group">
                <label htmlFor="price">Price in Near</label>
                {touched.price && errors.price && (
                  <p className="alert alert-danger">{errors.price}</p>
                )}

                <Field name="price" className="form-control" />
              </div>

              <div className="form-group">
                <label htmlFor="token_count">NFT count to be minted</label>
                {touched.token_count && errors.token_count && (
                  <p className="alert alert-danger">{errors.token_count}</p>
                )}

                <Field name="token_count" className="form-control" />
              </div>

              <div className="text-center">
                <button
                  type="submit"
                  className="btn btn-primary"
                  disabled={isSubmitting}
                >
                  Submit Form
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

export default SetNFTPrice
