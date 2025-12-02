struct ListNode* reverseKGroup(struct ListNode* head, int k){
    struct ListNode* tmp=head,*keep;
    short counter=k;
    while(counter--){
        if(tmp==0) return head;
        tmp=tmp->next;
    }
    counter=k;
    tmp=head;
    while(--counter){
        keep=tmp->next;
        tmp->next=keep->next;
        keep->next=head;
        head=keep;
    }
    tmp->next=reverseKGroup(tmp->next,k);
    return head;
}