class Solution {
    public int[] productExceptSelf(int[] nums) {
        ArrayList<Integer> products = getProducts(nums);
        ArrayList<Integer> productsBackwards = getProductsBackwards(nums);
        int[] answer = new int[nums.length];

        /* [1,2,3,4]
        [1, 2, 6, 24]
        [24,12,8,6] */
        int indexes = products.size() - 1;
        int val;

        for(int i = 0; i < products.size(); i++){
            if(i == 0){
                val = productsBackwards.get(indexes - 1);
            }
            else if(i == indexes){
                val = products.get(indexes - 1);
            }
            else{
                val = products.get(i - 1) * productsBackwards.get(indexes - i - 1);
            }
            answer[i] = val;
        }

        return answer;
    }

    private ArrayList<Integer> getProducts(int[] nums) {
        ArrayList<Integer> products = new ArrayList<>();
        int product = 1;
        for(int i = 0; i < nums.length; i++){
            product *= nums[i];
            products.add(product);
        }        

        return products;
    }

    private ArrayList<Integer> getProductsBackwards(int[] nums) {
        ArrayList<Integer> products = new ArrayList<>();
        int product = 1;
        for(int i = nums.length - 1; i > 0; i--){
            product *= nums[i];
            products.add(product);
        }    
        return products;
    }    
}