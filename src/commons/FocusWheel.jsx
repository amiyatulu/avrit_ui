import React, { useEffect } from "react"
import { useFormikContext } from "formik"
const FocusError = () => {
  const { errors, isSubmitting, isValidating } = useFormikContext()

  useEffect(() => {
    if (isSubmitting && !isValidating) {
      let keys = Object.keys(errors)
      if (keys.length > 0) {
        const selector = `[for=${keys[0]}]`
        // console.log(selector)
        const errorElement = document.querySelector(selector)
        // console.log(errorElement)
        if (errorElement) {
          errorElement.scrollIntoView()
        }
      }
    }
  }, [errors, isSubmitting, isValidating])

  return null
}

function SubmittingWheel(props) {
  const isSubmitting = props.isSubmitting
  const isValid = props.isVali
  const error = props.error
  // console.log(isSubmitting, isValid)
  if (isSubmitting && isValid) {
    return (
      <div>
        <br /> Submitting... ⌛
      </div>
    )
  } else {
    return <React.Fragment></React.Fragment>
  }
}

export { FocusError, SubmittingWheel }