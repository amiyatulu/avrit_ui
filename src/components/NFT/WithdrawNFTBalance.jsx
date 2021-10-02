import React, { useState, useContext } from "react"
import * as Yup from "yup"
import { Formik, Form, Field } from "formik"
import { useHistory, useParams } from "react-router-dom"
import ipfs from "../../commons/ipfs"
import { NearContext } from "../../commons/context/NearContext"
import { FocusError, SubmittingWheel } from "../../commons/FocusWheel"

function WithdrawNFTBalance(props) {
  // const [count, setCount] = useState(0);
  // const { id } = useParams()
  let history = useHistory()
  let { nearvar } = useContext(NearContext)
  const [errorThrow, setErrorThrow] = useState(false)

  return (
    <React.Fragment>
      <div className="container">
        <Formik
          initialValues={{}}
          validationSchema={Yup.object().shape({})}
          onSubmit={async (values, actions) => {
            try {
              //   values.countvariable = count
                const data = await nearvar.contract.withdraw_product_owner_incentives({})
              actions.setSubmitting(false)
              // console.log(data)
              // history.push(`/thankyou${data.mutationoutputname}`)
              // history.goBack()
              window.location.reload()
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
              <div className="text-center">
                <button
                  type="submit"
                  className="btn btn-primary"
                  disabled={isSubmitting}
                >
                  Withdraw Near
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

export default WithdrawNFTBalance
