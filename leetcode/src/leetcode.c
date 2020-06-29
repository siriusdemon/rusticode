#include <limits.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

int c_atoi(char *str) {
    char *p = str;
    int sign = 1;
    // skip the white space
    while (*p == ' ' && *p != '\0') p++;
    // judge the sign
    if (*p == '-') {
        sign = -1;
        p++;
    } else if (*p == '+') {
        p++;
    } else if (*p < '0' || *p > '9') {
        return 0;
    }
    // here we got very clean str!
    int ans = 0;
    while (*p != '\0' && *p >= '0' && *p <= '9') {
        int t = (*p - '0') * sign;
        if (ans > INT_MAX / 10 || (ans == INT_MAX / 10 && t > 7)) return INT_MAX;
        if (ans < INT_MIN / 10 || (ans == INT_MIN / 10 && t < -8)) return INT_MIN;
        ans = ans * 10 + t;
        p++;
    }
    return ans;
}

// https://leetcode-cn.com/problems/reverse-string/
void reverseString(char *s, int sSize) {

    // bounder check
    if (sSize == 0) { return; }

    char *head = s;
    char *tail = sSize + s - 1; // move point to last char
    char t = ' ';

    for (int i = 0; i < sSize / 2; i++) {
        t = *head;
        *head = *tail;
        *tail = t;    
        head++;
        tail--;
    }
}


// https://leetcode-cn.com/problems/container-with-most-water/
int maxArea(int* height, int heightSize){
    int li = 0;
    int ri = heightSize-1;
    int area = 0;
    int area_tmp = 0;
    int h = 0;
    
    while (li < ri) {
        h = (height[li] < height[ri]) ? height[li] : height[ri];
        area_tmp = (ri - li) * h;
        area = (area_tmp > area) ? area_tmp : area; 
        // move forward
        height[li] < height[ri] ? li++ : ri--;
    }
    return area;
}

// https://leetcode-cn.com/problems/trapping-rain-water/
// 解法：一次遍历，每次站到一个柱子上，看看有没有比它更高的柱子，有的话，从这根柱子到那
// 根更高的柱子之间都可以装满水，这时可以计算出可以装多少水。下一次，站到那根更高的柱子
// 上面。如果没有比目前更高的柱子，则扔掉脚下一块石头，再循环，直到当前柱子为0为止。
int trap(int* height, int heightSize){
    int cur = 0;
    int area = 0;

    while (cur < heightSize) {
        // 当前的位置高度，跳过零值
        int h = height[cur];
        if (h == 0) {
            cur++;
            continue;
        }

        // 寻找下一根柱子，满足条件：大于或者等于当前的柱子高度
        // 如果没找到，则当前柱子减1，继续找，直到当前柱子为0
        int next = -1;
        int next_max = -1;
        while (h > 0 && next == -1) {
            for (int i = cur+1; i < heightSize; i++) {
                if (height[i] >= h) {
                    next = i;
                    break;
                }
                if (height[i] > next_max) {
                    next_max = height[i];
                }
            }
            if (next == -1) {
                h = next_max == -1 ? 0 : next_max;
            }
        }

        if (h == 0) {
            return area;
        }

        // 来到这里，说明 h >= 0, next != -1
        int new_area = (next - cur - 1) * h;
        for (int i = cur+1; i < next; i++) {
            new_area -= height[i];
        }
        area += new_area;
        // update cur
        cur = next;
    }
    return area;
}

// https://leetcode-cn.com/problems/3sum/solution/
/**
 * Return an array of arrays of size *returnSize.
 * The sizes of the arrays are returned as *returnColumnSizes array.
 * Note: Both returned array and *columnSizes array must be malloced, assume caller calls free().
 */

// compare function for qsort
// int comp(const void* a,const void* b)
// {
//     return *(int*)a - *(int*)b;
// }

// int** threeSum(int* nums, int numsSize, int* returnSize, int** returnColumnSizes){
// }

int min(int a, int b, int c) {
    if ((a < b) && (a < c)) {
        return a;
    } else if ((b < a) && (b < c)) {
        return b;
    } else {
        return c;
    }
}

int dp(char* word1, char* word2, int i, int j) {
    if (i == -1) {
        return j + 1;
    } else if (j == -1) {
        return i + 1;
    } else if (word1[i] == word2[j]) {
        return dp(word1, word2, i-1, j-1);
    } else {
        return min(
            dp(word1, word2, i-1, j) + 1,
            dp(word1, word2, i, j-1) + 1,
            dp(word1, word2, i-1, j-1) + 1
        );
    }
}

int minDistance(char* word1, char* word2) {
    int len1 = strlen(word1);
    int len2 = strlen(word2);
    if (len1 == 0 || len2 == 0) {
        return len1 + len2;
    } else {
        return dp(word1, word2, len1-1, len2-1);
    }
}

// int minDistance_iter(char* word1, char* word2) {
//     int len1 = strlen(word1);
//     int len2 = strlen(word2);
// }

struct ListNode {
    int val;
    struct ListNode* next;
};

struct ListNode* listnode_new(int val) {
    struct ListNode* p = malloc(sizeof(struct ListNode));
    p->val = val; 
    p->next = NULL;
    return p;
}

struct ListNode* listnode_append(struct ListNode* list, int val) {
    struct ListNode* new = listnode_new(val);
    struct ListNode* p = list;
    while (p->next != NULL) {
        p = p->next;
    }
    p->next = new;
    return list;
}

// https://leetcode-cn.com/problems/merge-two-sorted-lists/
struct ListNode* mergeTwoLists(struct ListNode* l1, struct ListNode* l2){
    struct ListNode* p = NULL;
    struct ListNode* head = NULL;

    // init
    if (l1 != NULL && l2 != NULL) {
        if (l1->val <= l2->val) {
            p = l1;
            l1 = l1->next;
        } else {
            p = l2;
            l2 = l2->next;
        }
    } else if (l1 == NULL) {
        return l2;
    } else if (l2 == NULL) {
        return l1;
    }
    head = p;

    while (l1 != NULL || l2 != NULL) {
        if (l1 == NULL) {
            p->next = l2;
            p = l2;
            l2 = l2->next;
        } else if (l2 == NULL) {
            p->next = l1;
            p = l1;
            l1 = l1->next;
        } else {
            if (l1->val <= l2->val) {
                p->next = l1;
                p = l1;
                l1 = l1->next;
            } else {
                p->next = l2;
                p = l2;
                l2 = l2->next;
            }
        }
    }
    return head;
}

void printlist(struct ListNode* l) {
    struct ListNode* p = l;
    while (p != NULL) {
        printf("%2d ", p->val);
        p = p->next;
    }
    puts("");
}

// int main() {
//     printf("leetcode in C!\n");

//     // struct ListNode* l1 = listnode_new(1);
//     // l1 = listnode_append(l1, 3);
//     // l1 = listnode_append(l1, 5);
//     // printlist(l1);
    
//     // struct ListNode* l2 = listnode_new(1);
//     // l2 = listnode_append(l2, 2);
//     // l2 = listnode_append(l2, 4);
//     // printlist(l2);

//     struct ListNode* l1 = NULL; 
//     struct ListNode* l2 = listnode_new(0);

//     struct ListNode* l3 = mergeTwoLists(l1, l2);
//     printlist(l3);
//     return EXIT_SUCCESS;
// }

// https://leetcode-cn.com/problems/er-jin-zhi-zhong-1de-ge-shu-lcof/submissions/
// 二进制位移法
int hammingWeight(u_int32_t n) {
    int c = 0;
    while (n > 0) {
        c += (n & 1);
        n = n >> 1;
    }
    return c;
}