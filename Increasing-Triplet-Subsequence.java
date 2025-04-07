class Solution {
    public boolean increasingTriplet(int[] nums) {
        int first = Integer.MAX_VALUE;
        Integer second = Integer.MAX_VALUE;
        Integer maybeFirst = Integer.MAX_VALUE;
        for(int i = 0; i < nums.length; i++){
            if(nums[i] > second){
                return true;
            }

            if(nums[i] <= first){
                if(nums[i] < maybeFirst){
                    maybeFirst = nums[i];
                    continue;
                }
                if(nums[i] > maybeFirst){
                    first = maybeFirst;
                    second = nums[i];
                    maybeFirst = Integer.MAX_VALUE;
                }
            }

            if(nums[i] > first && nums[i] < second){
                second = nums[i];
                continue;
            }
        }
        return false;
    }
}