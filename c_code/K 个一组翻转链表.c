struct ListNode* reverseKGroup(struct ListNode* head, int k) {
	struct ListNode *pre = NULL;
	struct ListNode *start = NULL;
	struct ListNode *end = NULL;
	struct ListNode *nextS = head; //下一段指针
	struct ListNode *next = NULL; //当前指针下一结点
	struct ListNode *ret = NULL;
	if (k == 1) return head;
	int s = 0; //当前段序号
	while (nextS) {
		start =end= nextS;
		for (int i = 0; i < k-1; i++) {
			end = end->next;
			if (!end&&s==0) return  head;
			else if (!end) return ret;
		}
		if (s == 0) ret = end;
		pre = nextS = end->next;
		if (nextS != NULL) {
			for (int i = 0; i < k - 1; i++) {
				pre = pre->next;
				if (!pre) {
					pre = nextS;
					break;
				}
			}
		}
		while (start != nextS) {
			next = start->next;
			start->next = pre;
			pre = start;
			start = next;
		}
		s++;
	}
	return ret;
}