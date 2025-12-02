struct ListNode* reverseKGroup(struct ListNode* head, int k){
    
    if ( head == NULL || head->next == NULL ) return head;
    
    struct ListNode * newHead = (struct ListNode *) malloc(sizeof(struct ListNode));
    newHead->next = head;
    
    struct ListNode * rHead = head;
    struct ListNode * cNode = head;
    struct ListNode * sNode = head;
    struct ListNode * preNode = newHead;
    
    while ( true )
    {
        int expendable = 0;
        for ( ; expendable < k; ++expendable )
        {
            if ( cNode == NULL ) return newHead->next;
            cNode = cNode->next;
        }
        
        struct ListNode * tmpNode;
        while ( --expendable > 0 )
        {
            tmpNode = sNode->next;
            sNode->next = tmpNode->next;
            tmpNode->next = rHead;
            rHead = tmpNode;
        }

        preNode->next = rHead;
        preNode = sNode;
        rHead = sNode = cNode;
    }
    
    return newHead->next;
    
}