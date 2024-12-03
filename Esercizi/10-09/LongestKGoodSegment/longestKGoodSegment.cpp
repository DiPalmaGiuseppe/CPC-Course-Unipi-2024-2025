// https://codeforces.com/contest/616/problem/D?locale=en

#include <bits/stdc++.h>
using namespace std;

int32_t main() {
    int n, k;
    scanf("%d %d", &n, &k);

    if (k == n) {
        printf("%d %d", 1, n);
        return 0;
    }
    
    int vec[n];
    unordered_map<int, int> freq;
    int i = 0, j = 0, max_len = 0, max_i = 0, max_j = 0;

    while (j < n) {
        scanf("%d", &vec[j]);
        freq[vec[j]]++;
        
        while ((int)freq.size() > k) {
            freq[vec[i]]--;
            if (freq[vec[i]] == 0) {
                freq.erase(vec[i]);
            }
            i++;
        }

        if (j - i + 1 > max_len) {
            max_len = j - i + 1;
            max_i = i;
            max_j = j;
        }

        j++;
    }

    printf("%d %d", max_i + 1, max_j + 1);
    return 0;
}
