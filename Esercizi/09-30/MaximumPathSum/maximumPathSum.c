// https://www.geeksforgeeks.org/problems/maximum-path-sum/1

/*
struct Node
{
    int data;
    struct Node* left;
    struct Node* right;
    
    Node(int x){
        data = x;
        left = right = NULL;
    }
};
*/

class Solution {
public:
    int maxPathSum(Node* root)
    {
        pair<int, int> pair = rec_maxPathSum(root);
        int best = pair.first;
        int max_root = pair.second;
        if(root->left == NULL || root->right == NULL){ 
            return max(best, max_root);
        }
        return best;
    }
private:
    pair<int, int> rec_maxPathSum(Node *node){
        if(node == NULL)
            return  make_pair(INT_MIN,INT_MIN);
            
        
        pair<int, int> left = rec_maxPathSum(node->left);
        pair<int, int> right = rec_maxPathSum(node->right);
        
        int best = max({left.first, right.first});
        
        if(left.second != INT_MIN && right.second != INT_MIN)
            best = max(best,left.second + right.second + node->data);
        
        int max_;
        
        if(left.second == INT_MIN && right.second == INT_MIN)
            max_ = node->data;
        else
            max_ = node->data + max(left.second, right.second);
        
        return make_pair(best, max_);
    }
};