pub fn build_proverb(list: &[&str]) -> String {
   	let word=list.len()-1;
	let mut result= String::new();
   	if list.is_empty(){
   		return result
   		}

	for i in 0..word{

		result =result + &(format!("For want of a {} the {} was lost.",list[i],list[i+1])+"\n" );

		}
	result = result + &format!("And all for the want of a {}.",list[0]);

	result
}
