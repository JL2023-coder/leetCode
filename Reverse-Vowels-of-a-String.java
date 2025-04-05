class Solution {
    public String reverseVowels(String string) {
        StringBuilder s = new StringBuilder();
        for(int i = 0; i < string.length(); i++){
            s.append(string.charAt(i));
        }

        HashMap<Character, Integer> vowels = new HashMap();
        vowels.put('a', 1);
        vowels.put('e', 1);
        vowels.put('i', 1);
        vowels.put('o', 1);
        vowels.put('u', 1);

        ArrayList<Character> vowelsInString = new ArrayList();
        ArrayList<Integer> vowelsInStringIndex = new ArrayList();

        for(int i = 0; i < s.length(); i++){
            Character c = s.charAt(i);
            if(vowels.getOrDefault(Character.toLowerCase(c), 0) == 1){
                vowelsInString.add(c);
                vowelsInStringIndex.add(i);

            }
        }

        Collections.reverse(vowelsInStringIndex);

        int numVowels = vowelsInString.size();

        for(int i = 0; i < numVowels; i++){
            int index = vowelsInStringIndex.get(i);
            Character c = vowelsInString.get(i);
            s.setCharAt(index, c);
        }

        return s.toString();
    }
}