class Solution {
    public int lengthOfLongestSubstring(String s) {
        // Map to store character in string and its index
        HashMap<Character, Integer> map = new HashMap<Character, Integer>();

        // Longest substring
        int longest = 0;

        // current sub
        int current = 0;

        // count since last removal
        int indexLastRemoval = 0;

        for(int i = 0; i < s.length(); i++){
            current++;
            char c = s.charAt(i);
            if(map.containsKey(c) && map.get(c) >= indexLastRemoval){
                int toBeRemoved = map.get(c) - indexLastRemoval + 1;
                current -= toBeRemoved;
                indexLastRemoval = map.get(c) + 1;
            }
            if(current > longest){
                longest = current;
            }
            map.put(c, i);
        }

        return longest;
    }
}