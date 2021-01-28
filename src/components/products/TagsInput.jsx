import React from "react"
import styles from "./TagsInput.module.css"

const TagsInput = props => {
	const [tags, setTags] = React.useState(props.tags);
	const removeTags = indexToRemove => {
		setTags([...tags.filter((_, index) => index !== indexToRemove)]);
	};
	const addTags = event => {
		if (event.target.value !== "") {
			setTags([...tags, event.target.value.slice(0, -1)]);
            props.selectedTags([...tags, event.target.value.slice(0, -1)].join());
            props.setFieldValue(props.name, [...tags, event.target.value.slice(0, -1)].join())
			event.target.value = "";
		}
	};
	return (
		<div className={styles.tagsinput}>
			<ul id={styles.tags}>
				{tags.map((tag, index) => (
					<li key={index} className={styles.tag}>
						<span className={styles.tagtitle}>{tag}</span>
						<span className={styles.tagcloseicon}
							onClick={() => removeTags(index)}
						>
							x
						</span>
					</li>
				))}
			</ul>
			<input
				type="text"
				onKeyUp={event => event.key === "," ? addTags(event) : null}
				placeholder="Press comma to add tags"
			/>
		</div>
	);
};

export default TagsInput