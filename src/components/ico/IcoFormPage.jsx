import React, { useState, useContext } from "react"
import * as Yup from "yup"
import { Formik, Form, Field } from "formik"
import { useHistory, useParams } from "react-router-dom"
import ipfs from "../../commons/ipfs"
import { NearContext } from "../../commons/context/NearContext"
import { FocusError, SubmittingWheel } from "../../commons/FocusWheel"
import GetPrice from "./GetPrice"
import * as nearAPI from "near-api-js"
import ProgressBar from "./ProgressBar"
import "./IcoFormPage.css"
import PulseEffectTitle from "./PulseEffectTitle"
import { BigNumber } from "bignumber.js";


function IcoFormPage(props) {
  // const [count, setCount] = useState(0);
  // const { id } = useParams()
  let history = useHistory()
  let { nearvar, login, requestSignIn} = useContext(NearContext)
  const [errorThrow, setErrorThrow] = useState(false)
  const [numberOfTokens, setNumberOfTokens] = useState(1)
  let pw = BigNumber(10).pow(18)
  
  // let { count } = GetPrice(parseFloat(numberOfTokens) * Math.pow(10, 18))
  let { count } = GetPrice(BigNumber(numberOfTokens).times(pw))
  // console.log(count, 'count')
  const { utils } = nearAPI
  function handleChangeMethod(e) {
    // console.log(e)
    setNumberOfTokens(e.currentTarget.value)
  }
  return (
    <React.Fragment>
      <div className="container">
        <PulseEffectTitle/>
        <Formik
          initialValues={{
            numberOfTokens: "1",
          }}
          validationSchema={Yup.object().shape({
            numberOfTokens: Yup.number().required("Number Of Avrit Tokens is required"),
          })}
          onSubmit={async (values, actions) => {
            try {
              //   values.countvariable = count
              let attotokens = BigNumber(values.numberOfTokens).times(pw)
              // console.log(attotokens.toFixed(), "attotokens")
              //   const data = await nearvar.contract.add_liquidity({"tokens": parseInt(values.buy), "avrit_id": "dev-1616661269131-9185280"}, 95000000000000, 500)
              const data = await nearvar.contract.buy_tokens(
                { number_of_tokens: attotokens.toFixed() },
                95000000000000,
                count
              )
              //   actions.setSubmitting(false)
              console.log(data)
              // history.push(`/thankyou${data.mutationoutputname}`)
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
                {/* <p className="badge bg-info text-white">
                  1 atto Avrit = 10<sup>-18</sup> Avrit
                </p> */}
                <br />

                <label htmlFor="numberOfTokens" className="labelstyle">
                  Enter the Avrit token amount to buy:
                </label>
                {touched.numberOfTokens && errors.numberOfTokens && (
                  <p className="alert alert-danger">{errors.numberOfTokens}</p>
                )}

                <Field
                  name="numberOfTokens"
                  className="form-control"
                  onKeyUp={(e) => {
                    handleChangeMethod(e)
                    // let someValue = e.currentTarget.value
                  }}
                />
              </div>
              <div className="alert alert-hover color-1">
                Avrit token to buy: &nbsp;&nbsp;&nbsp;
                {/* {(new BN(numberOfTokens)).div(new BN("1000000000000000000")).toString()} */}
                {parseFloat(numberOfTokens)}
              </div>

              <div className="alert alert-hover color-4">
                Near fees: &nbsp;&nbsp;&nbsp;
                {utils.format.formatNearAmount(count)}
              </div>
              <div className="text-center">
                {login ? (
                  <div className="text-center">
                  <button
                    type="submit"
                    className="btn btn-hover color-7"
                    disabled={isSubmitting}
                  >
                    Buy Avrit Token
                  </button>
                </div>
                ) : (
                  <p className="btn btn-hover color-7" onClick={requestSignIn}>Log In to Buy <br/>Avrit Token</p>
                )}
              </div>
              
              <SubmittingWheel isSubmitting={isSubmitting} />
              <FocusError />
            </Form>
          )}
        </Formik>
        <br />
        <br />
        <div>
          <ProgressBar />
        </div>
      </div>
    </React.Fragment>
  )
}

export default IcoFormPage
